# geohashrs

This project provides geohash (https://en.wikipedia.org/wiki/Geohash) encoding and decoding based on geohash-rust (https://github.com/georust/geohash). You can use `geohash_encode(longitude, latitude, zoom_level)` to decode and `geohash_decode(geohashstring)` to decode.

Actually, this version is very early version, it is usable and more functionality will come.

## Installation 

```
pip install geohashrs
```

## Usage

Example:

```
>>> from geohashrs import geohash_encode, geohash_decode
>>> geohash_encode(5.4, 43.3, 10)
'spey6fs0v1'
>>> geohash_decode('spey6fs0v1')
[5.4, 43.3]
```
