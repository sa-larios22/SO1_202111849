package main

import (
	pb "client/proto"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/streadway/amqp"
	"golang.org/x/net/context"
	"google.golang.org/grpc"
)

func main() {
	port := "8081"
	http.HandleFunc("/", status)
	http.HandleFunc("/input", handleTweet)

	log.Printf("Starting Rest API on port %s", port)
	log.Fatal(http.ListenAndServe(":"+port, nil))

}

func status(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{
		"status": "Estado de la API Rest de Go: Activo",
	})
}

func handleTweet(w http.ResponseWriter, r *http.Request) {

	var tweet pb.TweetRequest

	err := json.NewDecoder(r.Body).Decode(&tweet)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	go func() {
		grpcServerAddress := os.Getenv("GRPC_SERVER_ADDRESS")

		if grpcServerAddress == "" {
			grpcServerAddress = "producer.tweets.svc.cluster.local:50051" // Valor por defecto
		}

		// Enviar a tweet al servidor gRPC
		err := sendToGRPCServer(grpcServerAddress, &tweet)
		if err != nil {
			log.Printf("Error sending tweet to gRPC server: %v", err)
			http.Error(w, "Error sending tweet to gRPC server", http.StatusInternalServerError)
			return
		} else {
			log.Printf("Tweet sent to gRPC server")
			w.Header().Set("Content-Type", "application/json")
			json.NewEncoder(w).Encode(map[string]string{
				"status": "Tweet enviado al servidor gRPC",
			})
		}

		// Enviar a RabbitMQ
		err = sendToRabbitMQ(&tweet)
		if err != nil {
			log.Printf("Error sending tweet to RabbitMQ: %v", err)
			http.Error(w, "Error sending tweet to RabbitMQ", http.StatusInternalServerError)
			return
		} else {
			log.Printf("Tweet sent to RabbitMQ")
		}
	}()

	w.WriteHeader(http.StatusOK)
	w.Write([]byte("Datos meteorológicos reenviados por gRPC y RabbitMQ"))

}

func sendToGRPCServer(address string, tweet *pb.TweetRequest) error {
	conn, err := grpc.Dial(address, grpc.WithInsecure())
	if err != nil {
		return err
	}
	defer conn.Close()

	client := pb.NewTweetServiceClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	res, err := client.SendTweet(ctx, tweet)

	if err != nil {
		return err
	}
	log.Printf("Response from gRPC server: %s", res.Status)

	return nil
}

func sendToRabbitMQ(tweet *pb.TweetRequest) error {
	rabbitMQURL := os.Getenv("RABBITMQ_URL")
	if rabbitMQURL == "" {
		rabbitMQURL = "amqp://user:Gh62vf3qHqIzFoI3@rabbitmq:5672/"
	}

	conn, err := amqp.Dial(rabbitMQURL)
	if err != nil {
		return fmt.Errorf("No se pudo conectar a RabbitMQ: %v", err)
	}

	defer conn.Close()

	ch, err := conn.Channel()
	if err != nil {
		return fmt.Errorf("No se pudo abrir un canal: %v", err)
	}
	defer ch.Close()

	q, err := ch.QueueDeclare(
		"tweets_queue", // nombre de la cola
		false,          // durable
		false,          // delete when unused
		false,          // exclusive
		false,          // no-wait
		nil,            // arguments
	)
	if err != nil {
		return fmt.Errorf("No se pudo declarar la cola: %v", err)
	}
	body, err := json.Marshal(tweet)
	if err != nil {
		return fmt.Errorf("Error al convertir los datos a JSON: %v", err)
	}

	err = ch.Publish(
		"",     // exchange
		q.Name, // routing key
		false,  // mandatory
		false,  // immediate
		amqp.Publishing{
			ContentType: "application/json",
			Body:        body,
		},
	)
	if err != nil {
		return fmt.Errorf("No se pudo publicar el mensaje en RabbitMQ: %v", err)
	}

	log.Println("📬 Mensaje enviado a RabbitMQ correctamente")
	return nil
}
