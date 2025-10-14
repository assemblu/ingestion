# Use Ubuntu as the base image
FROM ubuntu:22.04

# Set environment variables to avoid interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Update package list and install build tools
RUN apt-get update && \
    apt-get install -y build-essential cmake && \
    rm -rf /var/lib/apt/lists/*

# Create application directory
WORKDIR /app

# Copy the source code to the container
COPY . .

# Build the application
RUN mkdir -p build && \
    cd build && \
    cmake .. && \
    make

# Set the default command to run the executable
CMD ["./build/hello_world"]