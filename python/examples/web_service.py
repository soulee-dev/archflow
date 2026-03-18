"""기본 웹 서비스 아키텍처."""

from archflow import Diagram, Node, Cluster

with Diagram("Web Service Architecture", direction="LR") as d:
    client = Node("client", "Client")

    with Cluster("vpc", "AWS VPC"):
        lb = Node("lb", "Load Balancer")
        with Cluster("app_tier", "Application Tier"):
            api1 = Node("api1", "API Server 1")
            api2 = Node("api2", "API Server 2")
        with Cluster("data_tier", "Data Tier"):
            db = Node("db", "PostgreSQL")
            cache = Node("cache", "Redis")

    client >> lb
    lb >> api1
    lb >> api2
    api1 >> db
    api2 >> db
    api1 >> cache
    api2 >> cache

    d.save_svg("python/examples/web_service.svg")
