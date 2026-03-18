"""데이터 파이프라인 아키텍처."""

from archflow import Diagram, Node, Cluster

with Diagram("Data Pipeline", direction="LR") as d:
    with Cluster("sources", "Data Sources"):
        api = Node("api_log", "API Logs")
        click = Node("click", "Clickstream")
        db_cdc = Node("cdc", "DB CDC")

    with Cluster("ingest", "Ingestion"):
        kafka = Node("kafka", "Kafka")
        kinesis = Node("kinesis", "Kinesis")

    with Cluster("process", "Processing"):
        spark = Node("spark", "Spark")
        flink = Node("flink", "Flink")

    with Cluster("store", "Storage"):
        s3 = Node("s3", "S3 Data Lake")
        redshift = Node("redshift", "Redshift")

    with Cluster("serve", "Serving"):
        dashboard = Node("dashboard", "Dashboard")
        ml = Node("ml", "ML Models")

    api >> kafka
    click >> kinesis
    db_cdc >> kafka
    kafka >> spark
    kinesis >> flink
    spark >> s3
    flink >> s3
    s3 >> redshift
    redshift >> dashboard
    s3 >> ml

    d.save_svg("python/examples/data_pipeline.svg")
