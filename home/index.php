<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Web server home</title>
    <!-- import bootstrap -->
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-QWTKZyjpPEjISv5WaRU9OFeRpok6YctnYmDr5pNlyT2bRjXh0JMhjY6hW+ALEwIH" crossorigin="anonymous">
</head>

<body>
    <div class="container mt-5">
        <h1 class="mb-4">Welcome to the Web Server!</h1>
        <p>This is a simple web server home page.</p>
        <?php
        // Simple PHP test - let CGI handle headers
        echo "<h1>Hello from PHP!</h1>\n";
        echo "<p>Current time: " . date('Y-m-d H:i:s') . "</p>\n";
        echo "<p>Request method: " . ($_SERVER['REQUEST_METHOD'] ?? 'Unknown') . "</p>\n";
        echo "<p>Request URI: " . ($_SERVER['REQUEST_URI'] ?? 'Unknown') . "</p>\n";
        echo "<p>HTTP Host: " . ($_SERVER['HTTP_HOST'] ?? 'Unknown') . "</p>\n";
        echo "<p>Client IP: " . ($_SERVER['REMOTE_ADDR'] ?? 'Unknown') . "</p>\n";
        echo "<p>PHP is working! 🎉</p>\n";
        ?></div>
    </div>
</body>

</html>