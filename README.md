# Ingestion Project

> [!WARNING]
> Project gets periodic commits. I am mainly maintaining a local home server cluster where I also have my git setup there.

This is a Dockerized C++ "Hello, World!" application that demonstrates how to containerize a C++ application using Docker and CMake.

## Prerequisites

- Docker installed on your system
- Docker Compose (if using the recommended approach)

## Quick Start

To build and run the application container:

```bash
docker-compose up --build
```

This will:
1. Build the Docker image with the C++ application
2. Start the container
3. Output "Hello, World!"

## Alternative: Manual Docker Commands

If you prefer to use Docker directly without Docker Compose:

```bash
# Build the image
docker build -t ingestion-app .

# Run the container
docker run ingestion-app
```

## Stopping the Container

- If running in the foreground with `docker-compose up`, press `Ctrl+C` to stop
- If running in detached mode with `docker-compose up -d`, use `docker-compose down` to stop and remove containers

## Project Structure

- `src/main.cpp` - The C++ source code
- `CMakeLists.txt` - CMake build configuration
- `Dockerfile` - Docker image configuration
- `docker-compose.yml` - Docker Compose configuration