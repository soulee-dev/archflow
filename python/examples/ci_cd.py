"""CI/CD 파이프라인."""

from archflow import Diagram, Node, Cluster

with Diagram("CI/CD Pipeline") as d:
    dev = Node("dev", "Developer")

    with Cluster("ci", "CI Pipeline"):
        github = Node("github", "GitHub")
        build = Node("build", "Build")
        test = Node("test", "Test")
        scan = Node("scan", "Security Scan")

    with Cluster("cd", "CD Pipeline"):
        registry = Node("registry", "Container Registry")
        staging = Node("staging", "Staging")
        prod = Node("prod", "Production")

    monitor = Node("monitor", "Monitoring")

    dev >> github >> build >> test >> scan
    scan >> registry >> staging >> prod
    prod >> monitor

    d.save_svg("python/examples/ci_cd.svg")
