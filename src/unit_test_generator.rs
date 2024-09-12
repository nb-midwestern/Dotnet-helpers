use std::fmt::format;

use itertools::Itertools;

use crate::utils::{
    get_class_name::get_class_name, get_constructor_interfaces::get_constructor_interfaces,
    interface_to_name_transform::interface_to_name_transform,
};

pub fn run(file_text: String) -> String {
    let class_name = get_class_name(file_text.clone());
    let constructor_interfaces = get_constructor_interfaces(&file_text.clone());

    if let Some(class_name) = class_name {
        let include_transaction = constructor_interfaces
            .iter()
            .any(|i| i.contains("IUnitOfWork"));
        let test_class_name = format!("public class {class_name}Test");
        let build_sut = format!("private {class_name} BuildSystemUnderTest()");
        let build_mock = format!("private Mock<{class_name}> BuildMock()");
        let mock_object = build_mock_object(
            class_name.clone(),
            constructor_interfaces.clone(),
            include_transaction,
        );

        let transaction_mock = match include_transaction {
            true => {
                "private readonly Mock<IDbContextTransaction> _transaction = new();".to_string()
            }
            false => "".to_string(),
        };

        let mocks = constructor_interfaces
            .iter()
            .map(|interface| {
                format!(
                    "private readonly Mock<{}> _{} = new();",
                    interface.clone(),
                    interface_to_name_transform(&interface)
                )
            })
            .join("\n\t");

        return format!(
            r#"
{test_class_name}
{{    
    {mocks}
    {transaction_mock}
    

    {build_sut}
    {{
         return BuildMock().Object;
    }}

    {build_mock}
    {{
        {mock_object}
        return mock;
    }}

    [Fact]
    public async Task {class_name}_ShouldCompile()
    {{
        var _sut = BuildSystemUnderTest();
        Assert.NotNull(_sut);

    }}

}}
"#,
        );
    }
    todo!()
}

fn build_mock_object(
    class_name: String,
    interfaces: Vec<String>,
    include_transaction: bool,
) -> String {
    let mock_objects = interfaces
        .iter()
        .enumerate()
        .map(|(index, i)| {
            let transformed_name = format!("_{}.Object", interface_to_name_transform(i));
            if index < interfaces.len() - 1 {
                format!("{},", transformed_name)
            } else {
                transformed_name
            }
        })
        .join("\n\t\t");

    let transaction_setup = if include_transaction {
        format!(
            r#"
        _unitOfWork.Setup(uow => uow.BeginTransaction(It.IsAny<CancellationToken>()))
            .ReturnsAsync(_transaction.Object);

        _transaction.Setup(tran => tran.CommitAsync(It.IsAny<CancellationToken>()))
            .Returns(Task.CompletedTask);

        _transaction.Setup(tran => tran.RollbackAsync(It.IsAny<CancellationToken>()))
            .Returns(Task.CompletedTask);
        "#
        )
    } else {
        String::new()
    };

    format!(
        r#"
    {transaction_setup}
    var mock = new Mock<{class_name}>(
        {mock_objects}
    );
"#
    )
}
