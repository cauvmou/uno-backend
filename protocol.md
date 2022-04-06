# Game Structure

Protocol:

|Packet ID|Data|
|---------|----|
|u8|[u8;1023]

Cards (u8):

|Color|Number|
|-----|------|
|0x0-0xF|0x0-0x9|

|Color|Action|
|-|-|
|0x0-0xF|0xA-0xF|

## Server Requests

### Get username

|Packet ID|Username|
|---------|--------|
|0x01|[u8;1023]|

Username cannot have more than 20 characters, if more than 20 are sent to the server the string will just be cut off.

## Server Status

### Update game status

|Packet ID|Visible card|Deck|Stacked Cards|Direction|Current Player ID|
|---------|------------|----|-------------|---------|-|
|0x02|u8|[u8;256]|u8|0x0-0x1|u8

### User status

|Packet ID|Usernames|
|-|-|
|0x03|[u8;1023]
