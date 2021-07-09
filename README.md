# Mineswepttd

A RESTful Minesweeper server.

## Install

```shell
cargo install mineswepttd && mineswepttd
```

```shell
git clone https://github.com/pard68/mineswepttd && \
cd mineswepttd && \
cargo run
```

or

```shell
git clone https://github.com/pard68/mineswepttd && \
cd mineswepttd && \
cargo build --release && \
./target/release/mineswepttd
```

## Usage

Get a new random state:

`GET /new/<width>/<height>/<difficulty>`

### State

The board state has the following format (newlines are optional)

```text
10 10 10
currently-lately-sound-coral
00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00
```

The first line is made up of three integers; width, height, and difficulty
(difficulty refers to the number of mines on the board). The second line is the
seed for the board, a request to `/new` will return a human-friendly string,
however any unicode string is an acceptable seed. The remaining lines are a
series of two integer pairs, comprised of 1's and 0's. The first integer in the
each pair refers to the reveal state for the cell. The second integere in each
pair refers to the flag state for that cell.

Flag or unflag a cell:

`POST /flag/<x>/<y>?send_state=<true|false>`, passing the aforementioned state
as the body

Reveal a cell:

`POST /reveal/<x>/<y>?send_state=<true|false>`, passing the aforementioned state
as the body

### Flag and Reveal Responses

The `/flag` and `/reveal` endpoints reponses can be comprised of between one and
three parts. The first part, which will always be returned is what the board
looks like following a flagging or reveal. It looks like this:

```text
0000011100
000001F100
0000011100
0000000000
0000000000
0000111000
01122F1000
02....3100
14.....100
.......100
```

An integer refers to the number of neighboring mines for a _revealed_ cell. This
integer can be anything between 0 and 8. An `.` refers to an _unrevealed_,
_unflaaged_ cell. An `F` refers to a _flagged_ cell. An `M` (not shown above)
refers to a _revealed_ cell which contains a mine -- if an `M` is on the board,
than the game is over. Which brings us to the next potential part of the
response; the win/lose state.

```text
0000011100
000001F100
0000011100
0000000000
0000000000
0000111000
01122F1000
02..M.3100
14.....100
.......100
Lose!
```

If the line following the board is populated the game is over. The line will
contain one of two strings; `Win!` or `Lose!`. In the event that the third part
of the response is _requested_ than this line can also be blank -- `\n`. The
third potential part of a response is the board's state. Developers implementing
a frontend for `mineswepttd` can choose to either keep track of the game's state
in their own application or can request an updated state with each request. To
request a state, send the paramete `?send_state=true` with the `/flag` or
`/reveal` request.

## Development

A postman config can be found in the root of this project. It contains a request
for each of the endpoints currently available.
