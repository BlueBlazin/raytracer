from itertools import product

OP_TO_NAME = {'+': 'Add', '-': 'Sub', '*': 'Mul', '/': 'Div'}

HEADER = "use std::ops::{Add, Mul, Sub};\n"

MISC = """
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds."),
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}"""

TEMPLATE1 = """
impl {name}<{rhs}> for {ownership}Vec3 {{
    type Output = Vec3;

    fn {name_lower}(self, rhs: {rhs}) -> Self::Output {{
        Vec3(self.0 {op} rhs.0, self.1 {op} rhs.1, self.2 {op} rhs.2)
    }}
}}"""


TEMPLATE2 = """
impl {name}<{rhs}> for {ownership}Vec3 {{
    type Output = Vec3;

    fn {name_lower}(self, rhs: {rhs}) -> Self::Output {{
        Vec3(self.0 {op} rhs, self.1 {op} rhs, self.2 {op} rhs)
    }}
}}

impl {name}<{ownership}Vec3> for {rhs} {{
    type Output = Vec3;

    fn {name_lower}(self, rhs: {ownership}Vec3) -> Self::Output {{
        rhs {op} self
    }}
}}"""


def implement_scalar_op(op: str):
    name = OP_TO_NAME[op]
    res = []

    for own in ["&", ""]:
        res.append(TEMPLATE2.format(name=name,
                                    rhs="f64",
                                    ownership=own,
                                    name_lower=name.lower(),
                                    op=op))

    return res


def implement_vector_op(op: str):
    name = OP_TO_NAME[op]
    res = []

    for rhs_own, own in product(["&", ""], ["&", ""]):
        res.append(TEMPLATE1.format(name=name,
                                    rhs=f"{rhs_own}Vec3",
                                    ownership=own,
                                    name_lower=name.lower(),
                                    op=op))

    return res


def implement_ops():
    res = [HEADER, MISC]

    for op in OP_TO_NAME:
        res.extend(implement_vector_op(op))
        res.extend(implement_scalar_op(op))

    return "\n".join(res)


if __name__ == "__main__":
    print(implement_ops())
