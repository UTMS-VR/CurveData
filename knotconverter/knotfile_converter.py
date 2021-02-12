## Converting Knotplot ascii (raw) files to json compatible with DeformingKnot and vice versa
## S. Kaji (Feb. 2021)
## TODO: links with more than one component

import numpy as np
import argparse,os,glob,json


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='')
    parser.add_argument('--input', '-i', help='input directory containing *.raw or *.json')
    parser.add_argument('--output', '-o', default=".", help='output directory')
    parser.add_argument('--scale', '-s', default=1.0, help='scaling for Knotplot output')
    args = parser.parse_args()

    os.makedirs(args.output,exist_ok=True)
    if args.input is None:
        args.input = os.path.expanduser("~/AppData/Local/VirtualStore/Program Files (x86)/KnotPlot")

    for fname in glob.glob(os.path.join(args.input,"*"), recursive=True):
        fn, ext = os.path.splitext(fname)
        fn = os.path.basename(fn)
#        print(fname, fn, ext)
        if ext==".raw": # Knotplot file
            fn = os.path.join(args.output,fn+".json")
            print("processing... {} => {}".format(fname, fn))
            X = np.loadtxt(fname)
            m, M = X.min(), X.max()
            X = 2*(X-m)/(M-m)-1.0 # normalise to [-1,1]
            Y = []
            for x in X:
                Y.append({'x':x[0], 'y':x[1], 'z':x[2]})
            D = dict({'target': [dict({'points': Y, 'closed': True})]})
            with open(fn, 'w') as f:
                json.dump(D,f)
            
        elif ext==".json":  # DeformingKnot file
            fn = os.path.join(args.output,fn+".txt")
            print("processing... {} => {}".format(fname, fn))
            with open(fname, 'r') as f:
                Y = json.load(f)
            X = []
            for x in Y['target'][0]['points']:
                X.append([x['x'], x['y'],x['z']])
            np.savetxt(fn, args.scale*np.array(X),fmt='%.15f')
