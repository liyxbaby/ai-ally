
# AI Companion v1 API Documentation

## Introduction

The Companion API allows users to send and receive messages, manage companion settings, and retrieve various data related to the companion, user or backend.

## Base URL

The base URL for accessing the Companion API is `http://localhost:3000/api` or `http://<your_ip_address>:3000/api`

## Endpoints

### 1. Messages

#### 1.1 Get Messages

- **URL:** `/message`
- **Method:** `GET`
- **Description:** Retrieve a list of messages exchanged with the companion.
- **Parameters:**
  - `limit` (optional): The maximum number of messages to retrieve. Max is 50.
  - `offset` (optional): The offset for paginating through messages.
- **Response:**
  - Status: 200 OK
  - Body: Array of message objects.
- **Example Request:**
  ```http
  GET /message?limit=50&offset=0
  ```
- **Example Response:**
  ```json
  [
    {
      "id": 1,
      "ai": true,