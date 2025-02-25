#!/bin/bash
echo "🚀 Deploying Monitoring System for Hyper-X..."
docker run -d --name prometheus -p 9091:9091 prom/prometheus
docker run -d --name grafana -p 3000:3000 grafana/grafana
