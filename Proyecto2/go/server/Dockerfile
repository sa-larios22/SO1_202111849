# Building the binary of the App
FROM golang:1.19 AS build

WORKDIR /go/src

COPY . .

RUN go mod download

RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -a -installsuffix cgo -o app .

FROM alpine:latest as release

WORKDIR /app

COPY --from=build /go/src/.env ./.env

COPY --from=build /go/src/app .

EXPOSE 3001

ENTRYPOINT ["./app"]