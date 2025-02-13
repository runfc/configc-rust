use crate::errors::Error;

pub trait Manager {

    /*
     * ensure should ensure that a given value is implemented
     * or return an error, in case it can't.
     *
     * It MUST be indepotent. The usual pattern is to provide
     * the following:
     *
     *   1. Get the current value
     *   2. Check with the desired value
     *   3. Apply only if its different
     *
     * For a configuration management tool, is very important to
     * only apply changes when needed. As most of them might be
     * resource constly or even unsafe to apply after the system
     * reached a specific state.
     */
    fn ensure(&self) -> Result<(), Error>;

    /*
     * has_change get the actual value of the system and return
     * whether the value is different from the desired state (true)
     * or is equal (false).
     *
     * In case there's an error in getting the current state, a
     * Err() should be return instead. Doing so, provides the
     * caller the ability to differ from "all check it out"
     * instead of making assumption on facing of errors.
     */
    fn has_drifted(&self) -> Result<bool, Error>;
}
