import sys, getopt

def main(argv):
    inputfile = ''
    nfafile = ''
    debugconfig = '1'
    pattern_type = ''
    pattern_size = ''


    try:
        opts, args = getopt.getopt(argv,"hi:t:d:p:s:")
    except getopt.GetoptError:
        print('test.py -i <inputfile> -t <TAFile> -d <DebugConfig> -p <patternType> -s <patternSize>')
        sys.exit(2)
    for opt, arg in opts:
        if opt == '-h':
            print('test.py -i <inputfile> -t <TAFile> -d <DebugConfig> -p <patternType> -s <patternSize>')
            sys.exit()
        elif opt == '-i':
            inputfile = arg

        elif opt == '-t':
            nfafile = arg
        elif opt == '-d':
            debugconfig = arg

        elif opt == '-p':
            pattern_type = arg

        elif opt == '-s':
            pattern_size = arg

    out = ""
    out = out + 'input_dir = "data/graphs/' + inputfile + '/"\n'
    out = out + 'nfa_dir = "data/nfa/' + nfafile + '.csv"\n'
    out = out + 'debug =' + debugconfig + '\n'
    out = out + 'pattern_type = " ' + pattern_type+ '"\n'
    out = out +  'pattern_size = ' + pattern_size + '\n'
    print(out)

if __name__ == "__main__":
    main(sys.argv[1:])