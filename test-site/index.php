<?php
// Simple PHP test - let CGI handle headers
echo "<h1>Hello from PHP!</h1>\n";
echo "<p>Current time: " . date('Y-m-d H:i:s') . "</p>\n";
echo "<p>Request method: " . ($_SERVER['REQUEST_METHOD'] ?? 'Unknown') . "</p>\n";
echo "<p>Request URI: " . ($_SERVER['REQUEST_URI'] ?? 'Unknown') . "</p>\n";
echo "<p>HTTP Host: " . ($_SERVER['HTTP_HOST'] ?? 'Unknown') . "</p>\n";
echo "<p>Client IP: " . ($_SERVER['REMOTE_ADDR'] ?? 'Unknown') . "</p>\n";
echo "<p>PHP is working! 🎉</p>\n";