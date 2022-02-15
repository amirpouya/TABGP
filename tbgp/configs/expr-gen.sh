for ds in epl eu-email facebook
  for ta in taE ta1 ta2 ta3 ta4-2 ta5-2 ta6-2 ta7-2 ta8-2
    for patt in path2 cycle_two outstar_2 instar_2
      python config.gen.py -i $ds -t $ta -p $patt -s 2 -d 0 > conf.toml
      if $ta in ta2 ta7-2
      then
        echo "TTTTT"
      else
        echo "EEEEE"
      fi
  for ta in ta0-3 ta3-3 ta4-3 ta5-3 ta6-3
    for patt in path3 triangles_WOJ outstar_3 instar_3
          python config.gen.py -i $ds -t $ta -p $patt -s 3 -d 0 > conf.toml
