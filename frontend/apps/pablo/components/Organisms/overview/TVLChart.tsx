import { Skeleton, useTheme } from "@mui/material";
import { Chart } from "@/components";
import { DEFI_CONFIG } from "@/defi/config";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import { usePabloHistoricalTotalValueLocked } from "@/defi/hooks/overview/usePabloHistoricalTotalValueLocked";
import { useMemo } from "react";
import { pipe } from "fp-ts/function";
import * as O from "fp-ts/Option";
import { Range } from "@/defi/subsquid/overview";

export const TVLChart = () => {
  const theme = useTheme();
  const {
    chartSeries,
    setSelectedInterval,
    selectedInterval,
    durationLabels,
    isLoading,
  } = usePabloHistoricalTotalValueLocked();

  const intervals = DEFI_CONFIG.swapChartIntervals;

  const onIntervalChange = (symbol: string) => {
    pipe(
      intervals.find((interval) => interval.symbol === symbol),
      O.fromNullable,
      O.map((i) => setSelectedInterval(i.range as Range))
    );
  };

  const changeTextProps = useMemo(
    () =>
      pipe(
        chartSeries,
        O.fromPredicate((c) => c.length >= 2),
        O.bindTo("series"),
        O.bind("first", ({ series }) =>
          O.fromNullable(series.slice(0, 1).pop())
        ),
        O.bind("last", ({ series }) => O.fromNullable(series.slice(-1).pop())),
        O.bind("diff", ({ first, last }) =>
          first[1] + last[1] === 0
            ? O.none
            : O.some((100 * (last[1] - first[1])) / ((last[1] + first[1]) / 2))
        ),
        O.fold(
          () => ({
            changeText: "0.00%",
            changeTextColor: theme.palette.text.primary,
          }),
          ({ diff }) => ({
            changeText: `${diff.toFixed(2)}%`,
            changeTextColor:
              diff > 0 ? theme.palette.success.main : theme.palette.error.main,
          })
        )
      ),
    [
      chartSeries,
      theme.palette.error.main,
      theme.palette.success.main,
      theme.palette.text.primary,
    ]
  );

  const changeIntroText = useMemo(() => {
    switch (selectedInterval) {
      case "day":
        return `Past 24 hours`;
      case "week":
        return "Past week";
      case "month":
        return "Past month";
      case "year":
        return "Past year";
      case "all":
        return "All time";
    }
  }, [selectedInterval]);

  if (isLoading) {
    return <Skeleton variant="rounded" width="100%" height="512px" />;
  }

  return (
    <HighlightBox>
      <Chart
        height="100%"
        title="TVL"
        {...changeTextProps}
        changeIntroText={changeIntroText}
        AreaChartProps={{
          data: chartSeries,
          height: 330,
          shorthandLabel: "Change",
          labelFormat: (n: number) => n.toFixed(),
          color: theme.palette.featured.main,
        }}
        intervals={intervals.map((interval) => interval.symbol)}
        currentInterval={
          intervals.find((i) => i.range == selectedInterval)?.symbol
        }
        onIntervalChange={onIntervalChange}
        timeSlots={durationLabels}
      />
    </HighlightBox>
  );
};
