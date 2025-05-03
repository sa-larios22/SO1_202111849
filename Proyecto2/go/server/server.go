package main

import (
	"encoding/json"
	"log"
	"net/http"
	"os"
	pb "server/proto"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

type Tweet struct {
	Description string `json:"Description"`
	Country     string `json:"Country"`
	Weather     string `json:"Weather"`
}

var grpcClient pb.TweetServiceClient

func main() {
	// Conexión al servidor gRPC
	conn, err := grpc.NewClient(os.Getenv("GRPC_SERVER_ADDR"), grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("No se pudo conectar al servidor gRPC: %v", err)
	}
	defer conn.Close()
	grpcClient = pb.NewTweetServiceClient(conn)

	http.HandleFunc("/input", handleTweet)
	log.Println("🟢 API REST Go corriendo en :8081")
	log.Fatal(http.ListenAndServe(":8081", nil))
}

func handleTweet(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Método no permitido", http.StatusMethodNotAllowed)
		return
	}

	var tweet Tweet
	err := json.NewDecoder(r.Body).Decode(&tweet)
	if err != nil {
		http.Error(w, "Error en el cuerpo JSON", http.StatusBadRequest)
		return
	}

	// Enviar al gRPC
	resp, err := grpcClient.SendTweet(r.Context(), &pb.TweetRequest{
		Description: tweet.Description,
		Country:     tweet.Country,
		Weather:     tweet.Weather,
	})
	if err != nil {
		http.Error(w, "Error enviando al gRPC: "+err.Error(), http.StatusInternalServerError)
		return
	}

	json.NewEncoder(w).Encode(map[string]string{"status": resp.Status})
}
