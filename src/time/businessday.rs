pub enum BusinessDayConvention {
    // ISDA
    /**
     * Choose the first business day after the given holiday.
     */
    Following,

    /**
     * Choose the first business day after
     * the given holiday unless it belongs
     * to a different month, in which case
     * choose the first business day before
     * the holiday.
     */
    ModifiedFollowing,

    /**
     * Choose the first business day before
     * the given holiday.
     */
    Preceding,

    // NON ISDA
    /**
     * Choose the first business day before
     * the given holiday unless it belongs
     * to a different month, in which case
     * choose the first business day after
     * the holiday.
     */
    ModifiedPreceding,

    /**
     * Do not adjust.
     */
    Unadjusted,
}
