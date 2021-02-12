File converter for DeformingKnot 
==================
* Reuqires Python 3 with numpy: [Anaconda](https://anaconda.org) is recommended
* written by S. Kaji
* This small script is intended to convert various formats representing knots to JSON files used in DeformingKnot and vice versa.
Currently, only KnotPlot's ascii (raw) data files are supported.

# Usage

The following searches in _input_dir_ (and its subdirs) for files with extension ".raw" and ".json".

    > python knotfile_converter.py -i input_dir -o output_dir

- Those with ".raw" are assumed to be in the KnotPlot format which can be produced by ">save filename.raw raw" command in KnotPlot.
These files are converted to ".json" files.
- Those with ".json" are assumed to be in the DeformingKnot format. These files are converted to ".txt" files, which can be loaded by KnotPlot.


When executed without specifying input_dir, 
the script searches for the default KnotPlot directory for Winfows environments.

    > python knotfile_converter.py -o output_dir

The coverted ".json" files can be transfered to Quest by
[SideQuest](https://sidequestvr.com/).
- Go to "Manage files on the headset" on the upper menu (represented by a folder icon).
- Navigate to the "CurveData" directory.
- Put the json files there.

