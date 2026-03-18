"""마이크로서비스 아키텍처."""

from archflow import Diagram, Node, Cluster, connect

with Diagram("Microservices", direction="LR") as d:
    mobile = Node("mobile", "Mobile App")
    web = Node("web", "Web App")

    with Cluster("k8s", "Kubernetes Cluster"):
        gw = Node("gw", "API Gateway")

        with Cluster("svc", "Services"):
            auth = Node("auth", "Auth")
            user = Node("user", "User")
            order = Node("order", "Order")
            payment = Node("payment", "Payment")

        with Cluster("msg", "Messaging"):
            kafka = Node("kafka", "Kafka")

    with Cluster("storage", "Storage"):
        pg = Node("pg", "PostgreSQL")
        mongo = Node("mongo", "MongoDB")
        redis = Node("redis", "Redis")

    mobile >> gw
    web >> gw
    gw >> auth
    gw >> user
    gw >> order
    order >> payment
    order >> kafka
    connect(auth, redis)
    connect(user, pg)
    connect(order, pg)
    connect(payment, mongo)

    d.save_svg("python/examples/microservices.svg")
