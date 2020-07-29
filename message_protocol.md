# Message Protocol

## Request:
- Get a password entry:
  ```
  GET name
  EOF
  ```
- Add a new password entry:
  ```
  ADD name
  key:value;key:value;...
  EOF
  ```
- Delete a password entry:
  ```
  DELETE name
  EOF
  ```
- Create a new password file:
  ```
  CREATE filename
  masterkey
  EOF
  ```
- Open a existing password file:
  ```
  OPEN filename
  masterkey
  EOF
  ```
- Write a password to the clipboard
  ```
  CLIPBOARD name
  EOF
  ```
