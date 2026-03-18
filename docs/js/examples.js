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
    name: "GCP Architecture (Icons)",
    dsl: `title: GCP Data Pipeline
direction: LR

gcp:cloud-storage Data Lake >> gcp:bigquery Analytics
gcp:compute-engine App Server >> gcp:cloud-sql Database
gcp:compute-engine App Server >> gcp:cloud-storage Data Lake
gcp:bigquery Analytics >> gcp:looker Dashboard
gcp:gke Microservices >> gcp:cloud-run Functions

cluster:gcp:region us-central1 {
  gcp:compute-engine App Server
  gcp:cloud-sql Database
  gcp:cloud-storage Data Lake
  gcp:bigquery Analytics
  gcp:looker Dashboard
  gcp:gke Microservices
  gcp:cloud-run Functions
}

cluster:gcp:vpc Production VPC {
  gcp:compute-engine App Server
  gcp:cloud-sql Database
  gcp:gke Microservices
}`,
  },
  {
    name: "Kubernetes (Icons)",
    dsl: `title: Kubernetes Microservices
direction: LR

k8s:ingress Ingress >> k8s:service API Service >> k8s:deployment API Pods
k8s:service API Service >> k8s:config-map Config
k8s:deployment API Pods >> k8s:service DB Service >> k8s:stateful-set PostgreSQL
k8s:deployment API Pods >> k8s:service Cache Service >> k8s:deployment Redis
k8s:deployment API Pods >> k8s:secret Secrets

cluster:k8s:cluster Production Cluster {
  k8s:ingress Ingress
  k8s:service API Service
  k8s:deployment API Pods
  k8s:service DB Service
  k8s:stateful-set PostgreSQL
  k8s:service Cache Service
  k8s:deployment Redis
  k8s:config-map Config
  k8s:secret Secrets
}

cluster:k8s:namespace App Namespace {
  k8s:service API Service
  k8s:deployment API Pods
  k8s:config-map Config
  k8s:secret Secrets
}

cluster:k8s:namespace Data Namespace {
  k8s:service DB Service
  k8s:stateful-set PostgreSQL
  k8s:service Cache Service
  k8s:deployment Redis
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
    name: "AWS Architecture (Icons)",
    dsl: `title: AWS Web Architecture
direction: LR

aws:ELB Load Balancer >> aws:EC2 Web Server >> aws:RDS Database
aws:EC2 Web Server >> aws:S3 Static Assets
aws:EC2 Web Server >> aws:ElastiCache Cache

cluster:aws:region US East 1 {
  aws:ELB Load Balancer
  aws:EC2 Web Server
  aws:RDS Database
  aws:S3 Static Assets
  aws:ElastiCache Cache
}

cluster:aws:vpc Production VPC {
  aws:EC2 Web Server
  aws:RDS Database
  aws:ElastiCache Cache
}`,
  },
  {
    name: "Provider Clusters",
    dsl: `title: Multi-Cloud Setup
direction: TB

Users >> CDN >> API Gateway

API Gateway >> Auth Service
API Gateway >> App Server
App Server >> Primary DB
App Server >> Cache
App Server >> Queue
Queue >> Worker
Worker >> Primary DB

cluster:aws:region AWS Region {
  API Gateway
  Auth Service
  App Server
  Primary DB
  Cache
  Queue
  Worker
}

cluster:aws:vpc Production VPC {
  App Server
  Primary DB
  Cache
}

cluster:aws:subnet Private Subnet {
  Primary DB
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
