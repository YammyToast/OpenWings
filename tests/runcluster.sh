for i in $(seq 5)
do
    echo $i
    sleep 2
    python3 testserver.py &
done