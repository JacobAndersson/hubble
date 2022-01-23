wget https://stockfishchess.org/files/stockfish_14.1_linux_x64_avx2.zip -O stockfish-download.zip

unzip stockfish-download.zip

rm stockfish-download.zip
mv stockfish_14.1_linux_x64_avx2/stockfish_14.1_linux_x64_avx2 stockfish

chmod +x stockfish
rm -r stockfish_14.1_linux_x64_avx2/
