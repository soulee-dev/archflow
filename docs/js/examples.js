export const examples = [
  {
    name: "Basic Web Service",
    dsl: `title: Web Service
direction: LR

Client >> Load Balancer >> API Server 1 >> PostgreSQL
Load Balancer >> API Server 2 >> PostgreSQL
API Server 1 >> Redis
API Server 2 >> Redis

cluster AWS VPC {
  Load Balancer
  API Server 1
  API Server 2
  PostgreSQL
  Redis
}

cluster Application Tier {
  API Server 1
  API Server 2
}

cluster Data Tier {
  PostgreSQL
  Redis
}`,
  },
  {
    name: "CI/CD Pipeline",
    dsl: `title: CI/CD Pipeline
direction: TB

Developer >> GitHub >> Build >> Test >> Security Scan
Security Scan >> Container Registry >> Staging >> Production >> Monitoring

cluster CI Pipeline {
  GitHub
  Build
  Test
  Security Scan
}

cluster CD Pipeline {
  Container Registry
  Staging
  Production
}`,
  },
  {
    name: "Microservices",
    dsl: `title: Microservices Architecture
direction: LR

Mobile App >> API Gateway
Web App >> API Gateway
API Gateway >> Auth
API Gateway >> User
API Gateway >> Order
Order >> Payment
Order >> Kafka
Auth >> Redis
User >> PostgreSQL
Order >> PostgreSQL
Payment >> MongoDB

cluster Kubernetes Cluster {
  API Gateway
  Auth
  User
  Order
  Payment
  Kafka
}

cluster Services {
  Auth
  User
  Order
  Payment
}

cluster Messaging {
  Kafka
}

cluster Storage {
  PostgreSQL
  MongoDB
  Redis
}`,
  },
  {
    name: "Data Pipeline",
    dsl: `title: Data Pipeline
direction: LR

API Logs >> Kafka
Clickstream >> Kinesis
DB CDC >> Kafka
Kafka >> Spark
Kinesis >> Flink
Spark >> S3 Data Lake
Flink >> S3 Data Lake
S3 Data Lake >> Redshift
Redshift >> Dashboard
S3 Data Lake >> ML Models

cluster Data Sources {
  API Logs
  Clickstream
  DB CDC
}

cluster Ingestion {
  Kafka
  Kinesis
}

cluster Processing {
  Spark
  Flink
}

cluster Storage {
  S3 Data Lake
  Redshift
}

cluster Serving {
  Dashboard
  ML Models
}`,
  },
  {
    name: "Event-Driven",
    dsl: `title: Event-Driven Architecture
direction: LR

Web API >> Event Bus
Mobile API >> Event Bus
IoT Devices >> Event Bus
Event Bus >> Schema Registry
Event Bus >> Notification
Event Bus >> Analytics
Event Bus >> Billing
Event Bus >> Search Index

cluster Event Producers {
  Web API
  Mobile API
  IoT Devices
}

cluster Event Broker {
  Event Bus
  Schema Registry
}

cluster Event Consumers {
  Notification
  Analytics
  Billing
  Search Index
}`,
  },
];
