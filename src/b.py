import requests
import random
import json

url = "https://31pwr5t6ij.execute-api.eu-west-2.amazonaws.com"
id = "k.habara.aa@gmail.com QaNWIqumVxqbpOEjc4Qo-Q"

def select(problemName: str):
    req_body = {
        "id": id,
        "problemName": problemName,
    }
    res = requests.post(url + "/select", json=req_body)

def explore(plans: [str]):
    req_body = {
        "id": id,
        "plans": plans
    }
    res = requests.post(url + "/explore", json = req_body)
    res_body = json.loads(res.text)
    return res_body

def graph_to_connections(graph: [[int]]):
    # graph[i][j] := 部屋iのドアjから行ける部屋
    # connections := [{
    #    "from": {"room":int, "door":int},
    #    "to": {"room":int, "door":int}
    # },
    # ...]
    n_doors = 6
    n_rooms = len(graph)
    is_used = [[False] * n_doors for _ in range(n_rooms)]

    connections = []
    for i in range(len(graph)):
        for j in range(len(graph[i])):
            if is_used[i][j]:
                continue

            to = graph[i][j]
            rev = -1
            for k in range(n_doors):
                if graph[to][k] == i:
                    is_used[to][k] = True
                    rev = k
                    break
            assert rev != -1

            connections.append({
                "from": {"room": i, "door": j},
                "to": {"room": to, "door": rev}
            })
    return connections

# graph は普通の隣接リスト
def guess(rooms: [str], startingRoom, connections):
    req_body = {
        "id": id,
        "map": {
            "rooms": rooms,
            "startingRoom": startingRoom,
            "connections": connections
        }
    }
    res = requests.post(url + "/guess", json = req_body)
    res_body = json.loads(res.text)
    print(res_body)

def solve():
    problemName = "primus"
    select(problemName)

    n_doors = 6
    n_rooms = 6

    plans = [
        # 54文字のランダムな0~5の文字列
        "".join([str(random.randint(0, 5)) for _ in range(n_rooms * 18)])
    ]

    res = explore(plans)
    print(res["results"])
    print(res["queryCount"])

    rooms = [i for i in range(n_rooms)]
    startingRoom = 0
    graph = [[-1] * n_doors for _ in range(len(rooms))]

    current_room = startingRoom
    for i, room in enumerate(res["results"][0][1:]):
        graph[current_room][int(plans[0][i])] = room
        current_room = room

    # assert all doors are filled
    for i in range(len(graph)):
        print(graph[i])
    for i in range(len(graph)):
        for j in range(len(graph[i])):
            assert graph[i][j] != -1

    connections = graph_to_connections(graph)

    guess(rooms, startingRoom, connections)

import time
def main():
    while True:
        try:
            solve()
        except AssertionError:
            print("failed")
            continue

        time.sleep(0.5)

        break


if __name__ == "__main__":
    main()