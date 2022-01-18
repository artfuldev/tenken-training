FROM golang:1.17-alpine

ENV GO111MODULE=on

WORKDIR /bin/tenken
COPY ./go.mod ./go.mod
COPY ./go.sum ./go.sum
RUN go mod download
COPY ./main.go ./main.go

EXPOSE 3000

CMD go run main.go
