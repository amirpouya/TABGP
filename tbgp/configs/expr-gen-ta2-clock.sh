rm -f configs/sbatch-clock/*.sbatch
rm -f configs/toml-clock/*.toml
rm -f exprs-clock.sh
rm -rf expr-res-clock
mkdir expr-res-clock
mkdir configs/sbatch-clock/
mkdir configs/toml-clock/
clear
for ds in 0 4 8 16 32 64 128 256 512 1024
  do
    cp data/nfa/Ta2-Clock/ta2X.csv data/nfa/Ta2-Clock/ta2-$ds.csv
    ofile=data/nfa/Ta2-Clock/ta2-$ds.csv
    sed -i '' "s/XX/$ds/g" "$ofile"
    tfile="configs/toml-clock/$ds.toml"
    python3 configs/config.gen.py -i eu-email -t Ta2-Clock/ta2-$ds -p path2 -s 2 -d 0 > $tfile
    cp configs/sbatch-epl/sample-sbatch configs/sbatch-clock/$ds.sbatch
    echo "#SBATCH --output=expr-res-clock/$ds.csv" >> configs/sbatch-clock/$ds.sbatch
    echo "#SBATCH --job-name=clock$ds" >> configs/sbatch-clock/$ds.sbatch
    for i in {1..3}
      do
       echo   "./target/release/examples/baseline-clock $tfile">> configs/sbatch-clock/$ds.sbatch
       echo   "./target/release/examples/on-demand-clock $tfile">> configs/sbatch-clock/$ds.sbatch
       echo   "./target/release/examples/partial-clock $tfile">> configs/sbatch-clock/$ds.sbatch
      done

  done
