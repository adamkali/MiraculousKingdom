docker compose down

# Remove the specified containers
docker rmi miraculouskingdom-api miraculouskingdom-app

# Remove the specified volume
docker volume rm miraculouskingdom_mongodb_data
