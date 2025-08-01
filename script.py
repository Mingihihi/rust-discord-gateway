import json

with open("bro_im_dead.json", "r") as file:
    data = json.load(file)

for event in map(str, data):
    event = f"async fn {event.lower()}(&self, msg: Message) -> () {'{}'}"
    print(event)