for name in valence*; do
  newname=kanden"$(echo "$name" | cut -c8-)"
  mv "$name" "$newname"
done
