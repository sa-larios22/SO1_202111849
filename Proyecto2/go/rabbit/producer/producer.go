package main

import (
	"context"
	"encoding/json"
	"log"
	"net"
	"os"
	pb "producer/proto"
	"time"

	"github.com/streadway/amqp"
	"google.golang.org/grpc"
)

type server struct {
	ch    *amqp.Channel
	queue amqp.Queue
	pb.UnimplementedTweetServiceServer
}

func (s *server) SendWeather(ctx context.Context, in *pb.TweetRequest) (*pb.TweetResponse, error) {
	log.Println("Nuevo mensaje recibido de grpc-client")
	log.Printf("Datos recibidos:\nCountry: %s\nWeather: %s\nDescription: %s",
		in.Country, in.Weather, in.Description)

	body, err := json.Marshal(in)
	if err != nil {
		log.Printf("Error al serializar JSON: %v", err)
		return &pb.TweetResponse{Status: "error"}, err
	}

	err = s.ch.Publish(
		"",           // exchange
		s.queue.Name, // routing key
		false,
		false,
		amqp.Publishing{
			ContentType: "application/json",
			Body:        body,
			Timestamp:   time.Now(),
		})
	if err != nil {
		log.Printf("❌ Error al publicar en RabbitMQ: %v", err)
		return &pb.TweetResponse{Status: "error"}, err
	}

	log.Println("✅ Mensaje publicado en RabbitMQ correctamente")
	return &pb.TweetResponse{Status: "success"}, nil
}

func main() {
	rabbitURL := os.Getenv("RABBITMQ_URL")
	if rabbitURL == "" {
		rabbitURL = "amqp://user:Gh62vf3qHqIzFoI3@rabbitmq:5672/"
	}

	conn, err := amqp.Dial(rabbitURL)
	if err != nil {
		log.Fatalf("Error al conectar a RabbitMQ: %v", err)
	}
	log.Println("Conectado a RabbitMQ")
	defer conn.Close()

	ch, err := conn.Channel()
	if err != nil {
		log.Fatalf("Error al abrir canal: %v", err)
	}
	log.Println("✅ Canal abierto")
	defer ch.Close()

	queue, err := ch.QueueDeclare(
		"tweet_queue",
		true,
		false,
		false,
		false,
		nil,
	)
	if err != nil {
		log.Fatalf("Error al declarar cola: %v", err)
	}
	log.Println("Cola declarada: tweet_queue")

	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("Error al iniciar listener: %v", err)
	}

	s := grpc.NewServer()
	pb.RegisterTweetServiceServer(s, &server{ch: ch, queue: queue})

	log.Println("Servidor gRPC Producer-RabbitMQ escuchando en :50051")
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Error al servir: %v", err)
	}
}
