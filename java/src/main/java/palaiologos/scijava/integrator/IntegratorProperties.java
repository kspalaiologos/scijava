package palaiologos.scijava.integrator;

import palaiologos.scijava.SciFloat;

import java.util.Objects;

class IntegratorProperties {
    int precision;
    int degree;
    SciFloat a;
    SciFloat b;

    public IntegratorProperties(int precision, int degree, SciFloat a, SciFloat b) {
        this.precision = precision;
        this.degree = degree;
        this.a = a;
        this.b = b;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        IntegratorProperties that = (IntegratorProperties) o;
        return precision == that.precision &&
                degree == that.degree &&
                a.equals(that.a) &&
                b.equals(that.b);
    }

    @Override
    public int hashCode() {
        return Objects.hash(precision, degree, a.hashCode(), b.hashCode());
    }
}
