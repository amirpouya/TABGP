rm -f configs/sbatch/*.sbatch
rm -f configs/toml/*.toml
rm -f exprs.sh
rm -rf expr-res
mkdir expr-res
clear
for ds in epl #eu-email facebook
  do
    for patt in path2 cycle_two
      do
        for ta in taE ta1 ta3 ta4-2 ta5-2 ta6-2 ta8-2
          do
            tfile="configs/toml/$ds.$ta.$patt.toml"
            echo $tfile
            python3 configs/config.gen.py -i $ds -t $ta -p $patt -s 2 -d 0 > $tfile
            cp configs/sbatch/sample-sbatch configs/sbatch/$ds.$ta.$patt.sbatch
            echo "sbatch configs/sbatch/$ds.$ta.$patt.sbatch" >> exprs.sh
            echo "#SBATCH --output=expr-res/$ds.$ta.$patt.csv" >> configs/sbatch/$ds.$ta.$patt.sbatch
            for i in {1..3}
              do
                echo   "./target/release/examples/baseline $tfile">> configs/sbatch/$ds.$ta.$patt.sbatch
                echo   "./target/release/examples/on-demand $tfile">> configs/sbatch/$ds.$ta.$patt.sbatch
                echo   "./target/release/examples/partial $tfile">> configs/sbatch/$ds.$ta.$patt.sbatch
              done
          done
        for ta in ta2 ta7-2
        do
            tfile="configs/toml/$ds.$ta.$patt.toml"
            echo $tfile
            python3 configs/config.gen.py -i $ds -t $ta -p $patt -s 2 -d 0 > $tfile
            cp configs/sbatch/sample-sbatch configs/sbatch/$ds.$ta.$patt.sbatch

            echo "#SBATCH --output=expr-res/$ds.$ta.$patt.csv\n\n\n" >> configs/sbatch/$ds.$ta.$patt.sbatch
            echo "sbatch configs/sbatch/$ds.$ta.$patt.sbatch" >> exprs.sh
            for i in {1..3}
              do
                echo   "./target/release/examples/baseline-clock $tfile">> configs/sbatch/$ds.$ta.$patt.sbatch
                echo   "./target/release/examples/on-demand-clock $tfile">> configs/sbatch/$ds.$ta.$patt.sbatch
                echo   "./target/release/examples/partial-clock $tfile">> configs/sbatch/$ds.$ta.$patt.sbatch
              done
        done
    done

  for ta in ta0-3 ta3-3 ta4-3 ta5-3 ta6-3
    do
      for patt in path3 triangles_WOJ
          do
          tfile="configs/toml/$ds.$ta.$patt.toml"
            echo $tfile
            python3 configs/config.gen.py -i $ds -t $ta -p $patt -s 3 -d 0 > $tfile
            cp configs/sbatch/sample-sbatch configs/sbatch/$ds.$ta.$patt.sbatch
            echo "#SBATCH --output=expr-res/$ds.$ta.$patt.csv\n\n\n" >> configs/sbatch/$ds.$ta.$patt.sbatch
            echo "sbatch configs/sbatch/$ds.$ta.$patt.sbatch" >> exprs.sh

            for i in {1..3}
              do
                echo   "./target/release/examples/baseline $tfile">> configs/sbatch/$ds.$ta.$patt.sbatch
                echo   "./target/release/examples/on-demand $tfile">> configs/sbatch/$ds.$ta.$patt.sbatch
                echo   "./target/release/examples/partial $tfile">> configs/sbatch/$ds.$ta.$patt.sbatch
              done
      done
    done
  done