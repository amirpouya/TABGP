rm -f configs/sbatch-epl/*.sbatch
rm -f configs/toml-epl/*.toml
rm -f exprs-epl.sh
rm -rf expr-res-epl
mkdir expr-res-epl
mkdir configs/sbatch-epl/
mkdir configs/toml-epl/
clear
for ds in 0.1 0.2 0.3 0.4 0.5 0.6 0.7 0.8 0.9 1.0
  do
    for patt in path3 triangles_WOJ
      do
            ta='ta4-3'
            tfile="configs/toml-epl/$ds.$ta.$patt.toml"
            echo $tfile
            python3 configs/config.gen.py -i epl_dens/struct_density_efl_$ds -t $ta -p $patt -s 3 -d 0 > $tfile
            cp configs/sbatch-epl/sample-sbatch configs/sbatch-epl/$ds.$ta.$patt.sbatch
            echo "sbatch configs/sbatch-epl/$ds.$ta.$patt.sbatch" >> exprs-epl.sh
            echo "#SBATCH --output=expr-res-epl/$ds.$ta.$patt.csv" >> configs/sbatch-epl/$ds.$ta.$patt.sbatch
            echo "#SBATCH --job-name=epl$ds-$ta-$patt" >> configs/sbatch-epl/$ds.$ta.$patt.sbatch
            for i in {1..3}
              do
                echo   "./target/release/examples/baseline $tfile">> configs/sbatch-epl/$ds.$ta.$patt.sbatch
                echo   "./target/release/examples/on-demand $tfile">> configs/sbatch-epl/$ds.$ta.$patt.sbatch
                echo   "./target/release/examples/partial $tfile">> configs/sbatch-epl/$ds.$ta.$patt.sbatch
              done
        done

      for patt in path4 rectangle_woj
      do
            ta='ta4-4'
            tfile="configs/toml-epl/$ds.$ta.$patt.toml"
            echo $tfile
            python3 configs/config.gen.py -i epl_dens/struct_density_efl_$ds -t $ta -p $patt -s 4 -d 0 > $tfile
            cp configs/sbatch-epl/sample-sbatch configs/sbatch-epl/$ds.$ta.$patt.sbatch
            echo "sbatch configs/sbatch-epl/$ds.$ta.$patt.sbatch" >> exprs-epl.sh
            echo "#SBATCH --output=expr-res-epl/$ds.$ta.$patt.csv" >> configs/sbatch-epl/$ds.$ta.$patt.sbatch
            echo "#SBATCH --job-name=epl$ds-$ta-$patt" >> configs/sbatch-epl/$ds.$ta.$patt.sbatch
            for i in {1..3}
              do
                echo   "./target/release/examples/baseline $tfile">> configs/sbatch-epl/$ds.$ta.$patt.sbatch
                echo   "./target/release/examples/on-demand $tfile">> configs/sbatch-epl/$ds.$ta.$patt.sbatch
                echo   "./target/release/examples/partial $tfile">> configs/sbatch-epl/$ds.$ta.$patt.sbatch
              done
        done

    done
