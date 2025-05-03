import json
from random import randrange
from locust import HttpUser, between, task

class ReadFile():
    def __init__(self):
        self.data = []    
    
    # Load a random element from the list and remove it
    # If the list is empty, return None
    def getData(self):
        size = len(self.data)
        if size > 0:
            index = randrange(0, size - 1) if size > 1 else 0
            return self.data.pop(index)
        else:
            print("No hay más datos para enviar.")
            return None
    
    # Load the JSON file and parse it
    def loadFile(self):
        try:
            with open("./traffic/generated/weatherData.json", "r", encoding="utf-8") as file:
                self.data = json.loads(file.read())
        except Exception as e:
            print(f'Error: {e}')


class trafficData(HttpUser):
    wait_time = between(0.1, 0.9)
    reader = ReadFile()
    reader.loadFile()

    def on_start(self):
        print("On Start")
    
    @task
    def sendMessage(self):
        data = self.reader.getData()
        if data is not None:
            res = self.client.post("", json=data)
            response = res.json()
            print(response)
        else:
            print("Empty")
            self.stop(True)