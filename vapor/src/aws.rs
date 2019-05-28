use crate::protobuf::compute::Compute;

use rusoto_core::Region;
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Ec2Client, Filter, Reservation};

pub(crate) fn list_computes() -> Vec<Compute> {
    let client = Ec2Client::new(Region::UsWest2);

    let input = ec2_describe_input();
    let computes: Vec<Compute> = vec![];
    let mut result: Vec<Compute> = vec![];

    match client.describe_instances(input).sync() {
        Ok(resp) => match resp.reservations {
            Some(reservations) => {
                // println!("{}", reservations.len());
                result = normalize_reservations(reservations, computes);
            }
            None => println!("No instances to show"),
        },
        Err(_) => {
            println!("Error");
        }
    }

    result
}

// https://rusoto.github.io/rusoto/rusoto_ec2/struct.DescribeInstancesRequest.html
fn ec2_describe_input() -> DescribeInstancesRequest {
    let dry_run = Some(false);
    let filters = Some(vec![
        // https://rusoto.github.io/rusoto/rusoto_ec2/struct.Filter.html
        Filter {
            name: Some(String::from("instance-state-name")),
            values: Some(vec![String::from("running")]),
        },
    ]);
    let instance_ids = None;
    let max_results = None;
    let next_token = None;

    DescribeInstancesRequest {
        dry_run: dry_run,
        filters: filters,
        instance_ids: instance_ids,
        max_results: max_results,
        next_token: next_token,
    }
}

// https://rusoto.github.io/rusoto/rusoto_ec2/struct.DescribeInstancesResult.html
// https://rusoto.github.io/rusoto/rusoto_ec2/struct.Instance.html
fn normalize_reservations(
    reservations: Vec<Reservation>,
    mut result: Vec<Compute>,
) -> Vec<Compute> {
    for rev in reservations {
        let computes = rev.instances.unwrap();

        for comp in computes {
            result.push(Compute {
                id: comp.instance_id.unwrap(),
                compute_type: comp.instance_type.unwrap(),
            });
        }
    }

    result
}
