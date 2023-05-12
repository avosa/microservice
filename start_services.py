#!/usr/bin/env python3

import os
import subprocess
import signal
import socket
import sys

def check_kill_process(port):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    result = sock.connect_ex(('localhost', port))

    if result == 0:
        print("Port {} is open, trying to kill...".format(port))
        os.system("fuser -k {}/tcp".format(port))

check_kill_process(8080)
check_kill_process(8000)

print('Starting Gin microservice...')
subprocess.Popen(["go", "run", "main.go"], cwd='gin_microservice', stdout=sys.stdout, stderr=sys.stderr)

print('Starting Actix microservice...')
subprocess.Popen(["cargo", "run"], cwd='actix_microservice', stdout=sys.stdout, stderr=sys.stderr)

def signal_handler(sig, frame):
    print('Stopping services...')
    check_kill_process(8080)
    check_kill_process(8000)
    sys.exit(0)

signal.signal(signal.SIGINT, signal_handler)

# Wait indefinitely until Ctrl+C
signal.pause()
