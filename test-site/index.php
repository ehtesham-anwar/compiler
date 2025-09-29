<?php
echo "Content-Type: text/html\r\n\r\n";
echo "<h1>Hello from PHP!</h1>\n";
echo "<p>Current time: " . date('Y-m-d H:i:s') . "</p>\n";
echo "<p>Request method: " . $_SERVER['REQUEST_METHOD'] . "</p>\n";
echo "<p>Request URI: " . $_SERVER['REQUEST_URI'] . "</p>\n";
echo "<p>HTTP Host: " . $_SERVER['HTTP_HOST'] . "</p>\n";
echo "<p>Client IP: " . $_SERVER['REMOTE_ADDR'] . "</p>\n";
