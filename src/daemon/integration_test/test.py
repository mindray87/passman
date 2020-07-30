#!/usr/bin/env python3

import os
import socket
import unittest
import subprocess


class Tests(unittest.TestCase):

    def test_create(self):
        self.assertEqual(send_request(b'CREATE test_file'), 'OK')

    def test_add(self):
        self.assertEqual(send_request(b'CREATE test_file'), 'OK')
        self.assertEqual(send_request(b'ADD gmail username:name;password:passwd'), 'OK')

    def test_get(self):
        self.assertEqual(send_request(b'CREATE test_file'), 'OK')
        self.assertEqual(send_request(b'ADD gmail username:name;password:passwd'), 'OK')
        self.assertEqual(send_request(b'GET gmail'), 'OK username:name;password:passwd')

    def test_delete(self):
        self.assertEqual(send_request(b'CREATE test_file'), 'OK')
        self.assertEqual(send_request(b'ADD gmail username:name;password:passwd'), 'OK')
        self.assertEqual(send_request(b'GET gmail'), 'OK username:name;password:passwd')
        self.assertEqual(send_request(b'DELETE gmail'), 'OK')
        self.assertEqual(send_request(b'GET gmail'), 'ERR NotFound')

    def test_close(self):
        self.assertEqual(send_request(b'CREATE test_file'), 'OK')
        self.assertEqual(send_request(b'CLOSE test_file'), 'OK')
        self.assertEqual(send_request(b'GET gmail'), 'ERR NoOpenPasswordFile')

    def test_open_not_existing_file(self):
        self.assertEqual(send_request(b'OPEN test_file'), 'ERR FileDoesNotExist')

    def test_open(self):
        self.assertEqual(send_request(b'CREATE test_file'), 'OK')
        self.assertEqual(send_request(b'ADD gmail username:name;password:passwd'), 'OK')
        self.assertEqual(send_request(b'CLOSE test_file'), 'OK')
        self.assertEqual(send_request(b'GET gmail'), 'ERR NoOpenPasswordFile')
        self.assertEqual(send_request(b'OPEN test_file'), 'OK')
        self.assertEqual(send_request(b'GET gmail'), 'OK username:name;password:passwd')

    @classmethod
    def setUpClass(cls):
#           SetUp Deaemon
#         print("--- Start Tests ---")
#         proc1 = subprocess.Popen(os.getcwd() + "/target/debug/passman-daemon", shell=False)
#         os.popen(os.getcwd() + "/target/debug/passman-daemon")
#         p = subprocess.Popen(["sleep 2"])
#         p.wait()

    def tearDown(self):
        try:
            os.remove("/home/julian/.passman/test_file.pass")
        except FileNotFoundError:
            pass


def send_request(data):
    HOST = '127.0.0.1'  # The server's hostname or IP address
    PORT = 7878  # The port used by the server

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        s.sendall(data)
        s.shutdown(socket.SHUT_WR)
        return s.recv(1024).decode("utf-8")


if __name__ == '__main__':
    unittest.main()
