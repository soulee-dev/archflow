"""이벤트 드리븐 아키텍처."""

from archflow import Diagram, Node, Cluster, connect

with Diagram("Event-Driven Architecture", direction="LR") as d:
    with Cluster("producers", "Event Producers"):
        web = Node("web", "Web API")
        mobile = Node("mobile", "Mobile API")
        iot = Node("iot", "IoT Devices")

    with Cluster("broker", "Event Broker"):
        bus = Node("bus", "Event Bus")
        schema = Node("schema", "Schema Registry")

    with Cluster("consumers", "Event Consumers"):
        notify = Node("notify", "Notification")
        analytics = Node("analytics", "Analytics")
        billing = Node("billing", "Billing")
        search = Node("search", "Search Index")

    web >> bus
    mobile >> bus
    iot >> bus
    connect(bus, schema)
    bus >> notify
    bus >> analytics
    bus >> billing
    bus >> search

    d.save_svg("python/examples/event_driven.svg")
