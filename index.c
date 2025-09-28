#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>

#define PORT 80
#define BUFFER_SIZE 4096

char *get_local_path(char *domain)
{
    return "/Users/ehteshamanwar/Desktop/test-app";
}

void handle_php_request(int client_fd, const char *php_file)
{
    char cmd[512];
    char buffer[BUFFER_SIZE];
    FILE *php_output;

    snprintf(cmd, sizeof(cmd), "php %s", php_file);
    php_output = popen(cmd, "r"); // run php file and read output
    if (!php_output)
    {
        const char *error = "HTTP/1.1 500 Internal Server Error\n\nPHP execution failed.";
        send(client_fd, error, strlen(error), 0);
        return;
    }

    const char *header = "HTTP/1.1 200 OK\nContent-Type: text/html\n\n";
    send(client_fd, header, strlen(header), 0);

    while (fgets(buffer, sizeof(buffer), php_output))
    {
        send(client_fd, buffer, strlen(buffer), 0);
    }

    pclose(php_output);
}

int main()
{
    // Basic TCP server setup
    // integer file descriptors for server and client
    int server_fd, client_fd;
    // socket address structure
    struct sockaddr_in addr;
    // buffer to hold incoming request
    char request[BUFFER_SIZE];
    // create socket
    server_fd = socket(AF_INET, SOCK_STREAM, 0);

    // configure socket options
    addr.sin_family = AF_INET;         // IPv4
    addr.sin_addr.s_addr = INADDR_ANY; // bind to all interfaces
    addr.sin_port = htons(PORT);       // port number which is 80 for now

    bind(server_fd, (struct sockaddr *)&addr, sizeof(addr)); // bind socket to address
    listen(server_fd, 5);                                    // listen for incoming connections 5 is the backlog size of pending connections

    printf("C Web Server running on port %d\n", PORT);

    while (1)
    {
        // accept incoming connection
        client_fd = accept(server_fd, NULL, NULL);
        // read request
        // Explain memset:
        // The memset function is used to fill a block of memory with a particular value.
        // In this case, it is used to initialize the request buffer to zero before reading data into it.
        memset(request, 0, sizeof(request));
        // read data from client socket into request buffer
        // The read function reads data from a file descriptor (in this case, the client socket) into a buffer.
        // It returns the number of bytes read, or -1 if an error occurs.
        read(client_fd, request, sizeof(request));

        if (strstr(request, ".php"))
        {
            const char *response = "HTTP/1.1 200 OK\nContent-Type: text/plain\n\nHello from C server!";
            send(client_fd, response, strlen(response), 0);
        }

        close(client_fd);
    }
    return 0;
}
