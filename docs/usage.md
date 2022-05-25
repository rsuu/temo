# temo

```bash
te
    # like `te --ls`

te --add "work"
    # add a new task

te --sync
    # sync database

te 1 rm
    # delete task 1

te 1 info
    # show full info of 1

te 1 export
    # export 1 to json

te 1 import 1.json
    # import 1 from 1.json

te 1 done
    # mark 1 as done

te 1 project home.do.something
    # link 1 into a project

te '1..3' rm
    # delete task1 and task2 and task3

te '1.6.7' count
    #
    # task 1,6,7 count

te 's:p' count
    #
    # task count status:pending

te ''
    #
    #
te 'p:Home'
    #
    # task project:Home count

te 'p: ' count
    #
    # task project: count

te '-work'
    #
    # task -work count

te '+work'
    #
    # task +work count


te '/m.*ting/' count
    #
    #task /m..ting/ count

te 'p:Home && -work' count
    #
    #task project:Home and -work count

te 'p:Home || -work'
    #
    #task project:Home or -work count

te 'report.ls.filter'
    #
    #task show report.ls.filter

te 's:p && (p:Home || p:Garden)'
    #
    #status:pending and (project:Home or project:Garden)

te '(p: )'
    #
    #task '(project: )'


te gc
    # clear drop task

```
