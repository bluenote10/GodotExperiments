https://superuser.com/questions/1581256/how-to-overlay-multiple-images-with-position-and-scale-in-imagemagick/1581707#1581707

Solution:

```
convert circle.png inner.png -gravity Center -geometry 256x256+30+5 -composite -resize 64x64 output.png
```
