mkdir hw_2
cp -r src Cargo.toml README.md hw_2/
zip -r hw_2.zip hw_2/
rm -r hw_2/

#scp hw_2.zip freemadu@flip.engr.oregonstate.edu:
#rm -r hw_2.zip
