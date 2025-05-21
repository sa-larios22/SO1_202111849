from locust import HttpUser, task, between
import json
import random

class trafficData(HttpUser):
    wait_time = between(0.1, 0.9)

    def on_start(self):
        with open("./traffic/generated/weatherData.json", "r", encoding="utf-8") as file:
            self.payloads = json.load(file)
    
    @task
    def sendTweet(self):
        headers = {"Content-Type": "application/json"}
        payload = random.choice(self.payloads)
        self.client.post("/input", json=payload, headers=headers)