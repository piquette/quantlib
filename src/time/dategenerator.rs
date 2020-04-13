pub enum DateGenerator {
    /**
     * Backward from termination date to effective date.
     */
    Backward = 1,

    /**
     * Forward from effective date to termination date.
     */
    Forward = 2,

    /**
     * No intermediate dates between effective date and termination date.
     */
    Zero = 3,

    /**
     * All dates but effective date and termination date are taken to be on the third wednesday of their month (with forward
     * calculation.)
     */
    ThirdWednesday = 4,

    /**
     * All dates but the effective date are taken to be the twentieth of their month (used for CDS schedules in emerging
     * markets.) The termination date is also modified.
     */
    Twentieth = 5,

    /**
     * All dates but the effective date are taken to be the twentieth of an IMM month (used for CDS schedules.) The termination
     * date is also modified.
     */
    TwentiethIMM = 6,
}
