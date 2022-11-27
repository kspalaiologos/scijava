package palaiologos.scijava;

import java.lang.ref.Cleaner;

class CleanerSingleton {
    static final Cleaner CLEANER = Cleaner.create();
}
