echo 'Building the Docker image...'
sudo docker build -t hot_dog .

echo "Deploying the application..."
sudo docker stop hot_dog_container

# sudo docker run -it --rm --name hot_dog_container -p 35545:80 hot_dog ./web/server
sudo docker run -d --rm --name hot_dog_container -p 35545:80 hot_dog ./server