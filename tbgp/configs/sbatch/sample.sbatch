#!/bin/bash
#SBATCH --nodes=1                        # requests 3 compute servers
#SBATCH --ntasks-per-node=1              # runs 2 tasks on each server
#SBATCH --cpus-per-task=1                # uses 1 compute core per task
#SBATCH --time=4:00:00
#SBATCH --mem=32GB
#SBATCH --job-name=expr-test
#SBATCH --output=expr-test.out


