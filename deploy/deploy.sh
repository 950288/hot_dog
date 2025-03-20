echo 'Building the Docker image...'
sudo docker stop hot_dog_container
sudo docker build -t hot_dog .

echo "Deploying the application..."
# sudo docker run -it --rm --name hot_dog_container -p 35545:80 hot_dog ./server
sudo docker run -d --rm --name hot_dog_container -p 35545:80 hot_dog ./server