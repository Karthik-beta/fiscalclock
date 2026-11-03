"""Fiscal calendar engine: compute any fiscal calendar from a JSON definition.

Every computation happens in the Rust core. This module only converts Python
values to the core's JSON API and back, so results are identical to the Rust
crate and the npm package. The definition format and operations are specified
at https://github.com/Karthik-beta/fiscalclock/tree/main/spec.
"""

from __future__ import annotations

import datetime as _dt
import json as _json
import re as _re
from collections.abc import Mapping
from dataclasses import dataclass
from typing import Any, Literal, Optional, Union

from . import _fiscalclock as _core

__all__ = [
    "FiscalCalendar",
    "FiscalclockError",
    "Location",
    "Period",
    "call_json",
    "validate",
    "validate_json",
]

__version__ = "0.1.0"

Level = Literal["year", "quarter", "period", "week"]
DateLike = Union[_dt.date, str]
"""An exact ``datetime.date`` or an ISO ``YYYY-MM-DD`` string."""


class FiscalclockError(ValueError):
    """An error from the core, with its stable ``code`` and JSON ``path``.

    ``errors`` holds every error reported; validation can report several, and
    the other attributes describe the first.
    """

    def __init__(self, errors: list[dict[str, Any]]) -> None:
        self.errors = errors
        first = errors[0]
        self.code: str = first["code"]
        self.path: str = first["path"]
        self.message: str = first["message"]
        self.hint: Optional[str] = first.get("hint")
        self.details: Optional[dict[str, Any]] = first.get("details")
        text = f"{self.code} at {self.path or '(whole input)'}: {self.message}"
        if self.hint:
            text += f" (hint: {self.hint})"
        if len(errors) > 1:
            text += f"; and {len(errors) - 1} more"
        super().__init__(text)

    def __reduce__(self) -> tuple[Any, ...]:
        # The default rebuilds from ``args``, the message text, not the errors.
        return (type(self), (self.errors,), self.__dict__)


@dataclass(frozen=True)
class Period:
    """A fiscal period: the half-open interval ``[start, end_exclusive)``."""

    level: Level
    year: int
    index: Optional[int]
    start: _dt.date
    end_exclusive: _dt.date
    last_day: _dt.date
    days: int
    weeks: Optional[int]
    has_extra_week: bool

    @classmethod
    def _from_json(cls, v: dict[str, Any]) -> Period:
        return cls(
            level=v["level"],
            year=v["year"],
            index=v["index"],
            start=_dt.date.fromisoformat(v["start"]),
            end_exclusive=_dt.date.fromisoformat(v["end_exclusive"]),
            last_day=_dt.date.fromisoformat(v["last_day"]),
            days=v["days"],
            weeks=v["weeks"],
            has_extra_week=v["has_extra_week"],
        )


@dataclass(frozen=True)
class Location:
    """Where a date falls at every level."""

    date: _dt.date
    year: Period
    quarter: Period
    period: Period
    week: Period
    day_of_year: int
    day_of_quarter: int
    day_of_period: int
    day_of_week: int

    @classmethod
    def _from_json(cls, v: dict[str, Any]) -> Location:
        return cls(
            date=_dt.date.fromisoformat(v["date"]),
            year=Period._from_json(v["year"]),
            quarter=Period._from_json(v["quarter"]),
            period=Period._from_json(v["period"]),
            week=Period._from_json(v["week"]),
            day_of_year=v["day_of_year"],
            day_of_quarter=v["day_of_quarter"],
            day_of_period=v["day_of_period"],
            day_of_week=v["day_of_week"],
        )


_LONE_SURROGATE = _re.compile(r"[\ud800-\udfff]")


def _unsupported(value: Any) -> dict[str, str]:
    return {"unsupported_type": type(value).__name__}


def _json_text(value: Any, separators: Optional[tuple[str, str]] = None) -> str:
    # A value JSON cannot carry becomes {"unsupported_type": ...}, which the
    # core reports at its path. Input JSON cannot represent at all (a cycle, a
    # key that is not a string) becomes empty text, which the core rejects at
    # path "". A lone surrogate cannot cross into Rust as UTF-8, so it is
    # escaped as JSON.stringify escapes it, and the core reports it.
    try:
        text = _json.dumps(value, ensure_ascii=False, separators=separators, default=_unsupported)
    except (TypeError, ValueError, RecursionError):
        return ""
    return _LONE_SURROGATE.sub(lambda m: f"\\u{ord(m.group()):04x}", text)


def _definition_text(definition: Union[str, Mapping[str, Any]]) -> str:
    if isinstance(definition, str):
        return definition
    # Bytes, sets and other objects have no JSON form. JSON null makes the core
    # report that the definition is not an object, as it does for a list.
    if not isinstance(definition, (Mapping, list, tuple, int, float)):
        definition = None
    return _json_text(definition, separators=(",", ":"))


def _raise_if_errors(out: Any) -> Any:
    if isinstance(out, dict) and "errors" in out:
        raise FiscalclockError(out["errors"])
    return out


def _date_arg(value: Any, name: str) -> Any:
    # datetime.datetime is a subclass of date, and pandas Timestamp a subclass
    # of datetime, so this check must come before any date check.
    if isinstance(value, _dt.datetime):
        raise FiscalclockError(_json.loads(_core.timestamp_rejected(name))["errors"])
    if type(value) is _dt.date:
        return value.isoformat()
    # Strings and anything else go to the core, which reports INVALID_DATE or
    # INVALID_ARGUMENT for them, so the error matches the other languages.
    if isinstance(value, (str, int, float, bool)) or value is None:
        return value
    return {"unsupported_type": type(value).__name__}


def _period_arg(period: Union[Period, Mapping[str, Any]]) -> Any:
    if isinstance(period, Period):
        return {"level": period.level, "year": period.year, "index": period.index}
    # The core reads only these keys, so other values, such as dates, need not
    # serialise. Anything that is not a mapping goes to the core, which reports
    # it at "period".
    if isinstance(period, Mapping):
        return {k: period[k] for k in ("level", "year", "index") if k in period}
    return period


def validate(definition: Union[str, Mapping[str, Any]]) -> dict[str, Any]:
    """Validates a definition.

    Returns ``{"ok": True, "calendar_id": ..., "canonical": ...}`` or
    ``{"errors": [...]}``; it does not raise for an invalid definition.
    """
    return _json.loads(_core.validate(_definition_text(definition)))


def validate_json(definition: str) -> str:
    """Validates definition text and returns the core's JSON unchanged."""
    return _core.validate(definition)


def call_json(definition: str, op: str, args: str) -> str:
    """Runs one operation on JSON text and returns the core's JSON unchanged."""
    return _core.call(definition, op, args)


class FiscalCalendar:
    """A calendar built from a validated definition.

    Raises ``FiscalclockError`` if the definition is invalid. The definition is
    a JSON string or a mapping that serialises to the same JSON.
    """

    __slots__ = ("_cal",)

    def __init__(self, definition: Union[str, Mapping[str, Any]]) -> None:
        cal, errors = _core.Calendar.build(_definition_text(definition))
        if errors is not None:
            raise FiscalclockError(_json.loads(errors)["errors"])
        self._cal = cal

    @property
    def calendar_id(self) -> str:
        """The first 16 hex digits of the SHA-256 of the canonical form."""
        return self._cal.calendar_id

    @property
    def canonical_json(self) -> str:
        """The canonical form of the definition."""
        return self._cal.canonical_json

    def call_json(self, op: str, args: str) -> str:
        """Runs operation ``op`` with JSON arguments; returns the core's JSON unchanged."""
        return self._cal.call_json(op, args)

    def _call(self, op: str, args: dict[str, Any]) -> Any:
        out = _json.loads(self._cal.call_json(op, _json_text(args)))
        return _raise_if_errors(out)

    def locate(self, date: DateLike) -> Location:
        """The year, quarter, period and week containing ``date``."""
        return Location._from_json(self._call("locate", {"date": _date_arg(date, "date")}))

    def period(self, level: Level, year: int, index: Optional[int] = None) -> Period:
        """The period with coordinates ``(level, year, index)``; ``index`` is omitted for a year."""
        args: dict[str, Any] = {"level": level, "year": year}
        if index is not None:
            args["index"] = index
        return Period._from_json(self._call("period", args))

    def shift(self, period: Union[Period, Mapping[str, Any]], n: int) -> Period:
        """The period ``n`` steps away at the same level."""
        return Period._from_json(self._call("shift", {"period": _period_arg(period), "n": n}))

    def parent(self, period: Union[Period, Mapping[str, Any]]) -> Period:
        """The enclosing period one level up."""
        return Period._from_json(self._call("parent", {"period": _period_arg(period)}))

    def children(self, period: Union[Period, Mapping[str, Any]], level: Level) -> list[Period]:
        """The periods at ``level`` inside ``period``, in order."""
        out = self._call("children", {"period": _period_arg(period), "level": level})
        return [Period._from_json(p) for p in out]

    def years(self, first: int, last: int) -> list[Period]:
        """The fiscal years numbered ``first`` to ``last`` inclusive."""
        return [Period._from_json(p) for p in self._call("years", {"from": first, "to": last})]

    def format(self, period: Union[Period, Mapping[str, Any]], scheme: Optional[str] = None) -> str:
        """The label of ``period`` under ``scheme``, or the default scheme."""
        args: dict[str, Any] = {"period": _period_arg(period)}
        if scheme is not None:
            args["scheme"] = scheme
        return self._call("format", args)["label"]

    def parse(self, label: str) -> Period:
        """The period a label names. Raises with code ``AMBIGUOUS_LABEL`` when several match."""
        return Period._from_json(self._call("parse", {"label": label}))

    def dimension(self, start: DateLike, end_exclusive: DateLike) -> list[dict[str, Any]]:
        """One row per day in ``[start, end_exclusive)``, with dates as ISO strings."""
        return self._call(
            "dimension",
            {"start": _date_arg(start, "start"), "end_exclusive": _date_arg(end_exclusive, "end_exclusive")},
        )

    def dimension_csv(self, start: DateLike, end_exclusive: DateLike) -> str:
        """The rows of ``dimension`` as RFC 4180 CSV text."""
        return self._call(
            "dimension_csv",
            {"start": _date_arg(start, "start"), "end_exclusive": _date_arg(end_exclusive, "end_exclusive")},
        )

    def explain(self, date: DateLike) -> dict[str, Any]:
        """The rules applied to locate ``date``, step by step."""
        return self._call("explain", {"date": _date_arg(date, "date")})
