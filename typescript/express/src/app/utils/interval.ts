//
//  interval.ts
//  express
// 
//  Created by Wess Cope (me@wess.io) on 10/29/19
//  Copyright 2019 Wess Cope
//

export enum Interval {
  millisecondsPerSeconds  = 1_000,
  secondsPerMinute        = 60,
  minutesPerHour          = 60,
  hoursPerDay             = 24,
  daysPerWeek             = 7,
  monthsPerYear           = 12,
  second                  = (Interval.millisecondsPerSeconds),
  minute                  = (Interval.secondsPerMinute * Interval.second),
  hour                    = (Interval.minutesPerHour * Interval.minute),
  day                     = (Interval.hoursPerDay * Interval.hour),
  week                    = (Interval.daysPerWeek * Interval.day)
}
